import React, {useContext, useState} from "react";
import sanitizeHtml from "sanitize-html"
import {Container} from '@material-ui/core'
import {Theme} from "@material-ui/core/styles"
import {makeStyles, useTheme} from "@material-ui/styles"
import {v4 as uuidv4} from 'uuid'

import { Message, MessageState } from '../../components/commons/Message'
import { ChatDialog, ChatInput, Header } from '../../components'
import {createWASession, sendMessage as sendWAMessage} from '../../functions'
import Context from '../../Context'

// Default max_attempt is 1. Once for re-using session_id. Another for create new session_id after expired.
const MAX_ATTEMPT = process.env.REACT_APP_RETRY_SEND?parseInt(process.env.REACT_APP_RETRY_SEND):1

const useStyle = makeStyles((theme: Theme) => ({
    root: {
        backgroundColor: theme.palette.background.default
    }
}))

export default function ChatPage() {
    let theme = useTheme() as Theme
    let style = useStyle(theme)
    let [sessionId, setSessionId] = useState<string>()
    let [messages, setMessages] = useState<Message[]>([])
    let ctx = useContext(Context)

    const sendMessage = async (msg: string) => {
        // add message to chat screen first so user see that the text is sent
        // We probably need mechanism to ensure the text is received by server
        let inputMsg = {uuid: `m${uuidv4()}`, message: msg, type: "self", timestamp: new Date(), state: MessageState.sending}
        messages = messages.concat(inputMsg)
        setMessages(messages)

        try {
            // Fix #2
            // if (!sessionId) {
            //     let response = await createWASession()
            
            //     if (response.status === 201) {
            //         setSessionId(response.result.session_id)
                    
            //         sessionId = response.result.session_id
            //         } 
            // }

            for (let retry = 0; retry <= MAX_ATTEMPT; retry++) {
                inputMsg.state = MessageState.sending
                let response

                if (sessionId) {
                    // reuse existing sessionId
                    response = await sendWAMessage({
                        sessionId,
                        message: msg,
                        sourceLang: ctx.locale.substring(0, 2)
                    })
                } else {
                    // establish new session then send a message immediately
                    response = await sendWAMessage({
                        message: msg,
                        sourceLang: ctx.locale.substring(0, 2)
                    })
                }
                    
                if (response.status === 200) {
                    if (sessionId !== response.sessionId) {
                        // Server return new session
                        setSessionId(sessionId)
                        // temporary put sessionId onto existing one so we don't have to wait for next render
                        sessionId = response.sessionId
                    }
                    inputMsg.state = MessageState.succeed
                    let wa_output = response.result.output
                    let wa_context = response.result.context
                    let wa_response = wa_output.generic

                    
                    let replied: Message[] = wa_response.map((r: any) => {
                        switch (r.response_type) {
                            case "text":
                                return {
                                    uuid: `m${uuidv4()}`,
                                    message: sanitizeHtml(r.text),
                                    type: "Watson",
                                    timestamp: new Date(),
                                    context: wa_context,
                                    state: MessageState.succeed
                                }
                            case "suggestion":
                                return {
                                    uuid: `m${uuidv4()}`,
                                    message: r.title,
                                    type: "Watson",
                                    timestamp: new Date(),
                                    context: wa_context,
                                    state: MessageState.succeed,

                                    suggestions: r.suggestions.map((s: any) => {
                                        return {
                                            uuid: `su${uuidv4()}`,
                                            label: s.label,
                                            intents: s.value.input.intents
                                        }
                                    })
                                }
                            case "search":
                                return {
                                    uuid: `m${uuidv4()}`,
                                    message: r.header,
                                    type: "Watson",
                                    timestamp: new Date(),
                                    context: wa_context,
                                    state: MessageState.succeed,

                                    results: r.results.map((sr:any) => {
                                        let url = undefined
                                        if (sr.result_metadata && sr.result_metadata.source && sr.result_metadata.source.link) {
                                            url = sr.result_metadata.source.link.url
                                        } else if (sr.url) {
                                            url = sr.url
                                        }
                                        return {
                                            uuid: `se${uuidv4()}`,
                                            title: sanitizeHtml(sr.title),
                                            highlight: sanitizeHtml(sr.highlight.body[0]),
                                            url: url?sanitizeHtml(url):undefined
                                        }
                                    })
                                }
                            case "option":
                                return {
                                    uuid: `m${uuidv4()}`,
                                    message: r.title,
                                    type: "Watson",
                                    timestamp: new Date(),
                                    context: wa_context,
                                    state: MessageState.succeed,

                                    suggestions: r.options.map((o: any) => ({
                                        uuid: `op${uuidv4()}`,
                                        label: o.label
                                    }))
                                }
                            default:
                                return {
                                    uuid: `m${uuidv4()}`,
                                    message: "Unsupported type of response. Please contact admin.",
                                    type: "Watson",
                                    timestamp: new Date(),
                                    state: MessageState.succeed,
                                }
                        }
                    })

                    messages = messages.concat(...replied)

                    setMessages(messages)
                    break
                } else {
                    console.error("Fail to send message to WA server. Wait for 1 second to retry.")
                    await new Promise((resolve) => setTimeout(resolve, 1000))
                    let response = await createWASession()

                    if (response.status === 201) {
                        setSessionId(response.result.session_id)
                        // temporary put sessionId onto existing one so we don't have to wait for next render
                        sessionId = response.result.session_id
                    }
                }
            }
            
            if (inputMsg.state !== MessageState.succeed) {
                inputMsg.state = MessageState.failed
                setMessages([...messages])
                console.error("Max number of retry attempted without succeed")
            }
            
        } catch (err) {
            // some error occur, either can't establish session or fail to send WA msg
            inputMsg.state = MessageState.failed
            setMessages([...messages])
            console.error(err)
        }
    }

    return (
        <Container className={style.root}>
            <Header />
            <ChatDialog style={{height: "70vh", overflow: "scroll", padding: "0 0 0 0", margin: "0 0 0 0"}} messages={messages} onChoose={sendMessage}/>
            <ChatInput style={{height: "18vh"}} onSubmit={(msg) => {sendMessage(msg)}}/>
        </Container>
    )
}
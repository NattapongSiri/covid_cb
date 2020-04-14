import React, {useEffect, useRef} from "react"

import _debounce from 'lodash.debounce'
import {Container} from '@material-ui/core'

import ChatMessage from './ChatMessage'
import {Message} from './commons/Message'

export default function ChatDialog({messages, style, onChoose}: {style?: React.CSSProperties, messages: Message[], onChoose?: (msg: string) => void}) {

    // reference point for scrolling. It currently on bottom of this component
    const bottom_anchor = useRef<HTMLDivElement>(null)

    // utitlity function to scroll to bottom of this component
    const scrollToBottom = () => {
        if (bottom_anchor && bottom_anchor.current) {
            bottom_anchor.current.scrollIntoView({ block: 'end', behavior: 'smooth' });
        } else {
            console.error("Bottom anchor reference is missing")
        }
    }

    // scroll to bottom when screen resize
    useEffect(() => {
        const debouncedHandleResize = _debounce(scrollToBottom, 80)
    
        window.addEventListener('resize', debouncedHandleResize)
    
        return () => {
            window.removeEventListener('resize', debouncedHandleResize)
        }
    })

    // always scroll to bottom when component is re-render
    useEffect(() => scrollToBottom())

    return (
        <Container style={style}>
            {messages.map((m)=> {
                if (m.results) {
                    // search result type of reply
                    return (
                        <React.Fragment key={m.uuid.substring(1)}>
                            <ChatMessage key={m.uuid} message={m} onChoose={onChoose} />
                            {m.results.map((r) => (
                                <ChatMessage key={r.uuid} 
                                    message={{
                                        uuid: r.uuid,
                                        message: r.highlight,
                                        type: "Watson",
                                        timestamp: new Date(),
                                        reference: r.url
                                    }} />
                            ))}
                        </React.Fragment>
                    )
                } else {
                    if (m.message.startsWith("<a ")) {
                        // link to be preview message
                        let urlIdx = m.message.toLowerCase().indexOf("href=\"") + 6
                        let urlEndIdx = m.message.indexOf("\"", urlIdx + 1)
                        
                        // not empty href attribute
                        if (urlIdx < urlEndIdx) {
                            let url = new URL(m.message.substring(urlIdx, urlEndIdx))
                            m.previewUrl = url
                        }

                        return <ChatMessage key={m.uuid} message={m} onChoose={onChoose}/>
                    } else if (m.message.startsWith("http")) {
                        let url = new URL(m.message)
                        m.message = url.hostname
                        m.previewUrl = url
                        return <ChatMessage key={m.uuid} message={m} onChoose={onChoose}/>
                    } else {
                        // simple reply, may have some option for user to pick 
                        return <ChatMessage key={m.uuid} message={m} onChoose={onChoose}/>
                    }
                }
            })}
            {/* div below is used an anchor to scroll to bottom */}
            <div style={{width:0, height: 0, display: "block"}} ref={bottom_anchor}/>
        </Container>
    )
}
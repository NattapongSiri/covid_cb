import React, {useEffect, useState} from 'react'
import {useTranslation} from 'react-i18next'
import {Container, InputAdornment, OutlinedInput, ThemeProvider, useTheme} from '@material-ui/core'
import {Send} from '@material-ui/icons'
import {makeStyles} from '@material-ui/styles'

import {ContainerClass, InputClass} from './ChatInput-style'
import STT, {STT_State} from '../Speech2Text'
import {useDebouncedState} from '../commons/utilities'

const useContainerClass = makeStyles(ContainerClass)
const useInputClass = makeStyles(InputClass)

enum InputMethod {
    Keyboard,
    Voice
}

export default function ChatInput({style, locked = false, onSubmit}: {style?: React.CSSProperties, locked?: boolean, onSubmit?: (msg: string) => void}) {
    const theme = useTheme()
    const {t} = useTranslation("chatDialog")
    const containerClass = useContainerClass()
    const inputClass = useInputClass()
    let [composing, setComposing] = useState("")
    let [readyToSend, setReadyToSend] = useState(false)
    let [method, setMethod] = useState(InputMethod.Keyboard)
    let debouncedComposing = useDebouncedState(composing, 100)

    useEffect(() => {
        if (readyToSend && debouncedComposing.trim().length > 0 && onSubmit) {
            onSubmit(debouncedComposing)
            setReadyToSend(false)
            setComposing("")
        }
    }, [debouncedComposing, readyToSend, onSubmit])
    
    return (
        <ThemeProvider theme={theme}>
            <Container className={containerClass.root} style={style}>
                <OutlinedInput className={inputClass.root} 
                    disabled={locked}
                    placeholder={t("inputHelper")} 
                    onChange={(e)=>setComposing(e.target.value)} 
                    onKeyDown={(e) => {
                        // enter key press
                        if (e.key === 'Enter' || e.keyCode === 13) {
                            setReadyToSend(true)
                        }
                    }}
                    value={composing} 
                    endAdornment={method === InputMethod.Keyboard && composing.trim().length > 0 &&
                        <InputAdornment 
                            position="end" 
                            style={{cursor: "pointer"}} 
                            onClick={() => {
                                setReadyToSend(true)
                            }}
                        >
                            <Send/>
                        </InputAdornment>
                    }
                />
                <STT 
                    onData={(txt) => setComposing(txt)} 
                    onStateChange={(state, prev)=>{
                        if (state === STT_State.start) {
                            setMethod(InputMethod.Voice)
                        } else if(state === STT_State.ready && prev === STT_State.start) {
                            // case when user click to force stop recognition
                            setMethod(InputMethod.Keyboard)
                            console.log("Auto submit user input")

                            // auto submit the recognized text
                            setReadyToSend(true)
                        }
                    }
                }/>
            </Container>
        </ThemeProvider>
    )
}
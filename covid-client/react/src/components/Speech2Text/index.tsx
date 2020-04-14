/// <reference lib="dom" />

import React, {useContext, useState, useEffect} from 'react'
import {Mic} from '@material-ui/icons'
import {useTheme} from '@material-ui/styles'
import useStyle from './Speech2Text-style'
import Dot from '../Spinner/Dot'

import Context from '../../Context'

export enum STT_State {
    undefined,
    start,
    ready
}

var recognition: any;

export default function Speech2Text({
    onClick, 
    onData,
    onStateChange
}: {
    onClick?: (e?: React.MouseEvent, state?: STT_State) => void, 
    onData?: (data: string) => void,
    onStateChange?: (state: STT_State, prevState?: STT_State) => void
}) {
    let [state, setState] = useState(STT_State.undefined)
    let ctx = useContext(Context)

    // Initialize component's speech recognition only once
    useEffect(() => {
        var constraints = { audio: true }; 
        navigator.mediaDevices.getUserMedia(constraints).then(() => {
            console.log("Mic is available")

            // Initialize SpeechRecognition
            if (window.SpeechRecognition === undefined) {
                const {webkitSpeechRecognition} = (window as any)
                window.SpeechRecognition = webkitSpeechRecognition
            }
            
            var SpeechRecognition = (window as any).SpeechRecognition

            // no standard SpeecRecognition nor webkitSpeechRecognition available
            if (SpeechRecognition) {
                recognition = new SpeechRecognition()
                recognition.interimResults = true;
            } else {
                console.log("SpeechRecognition is not supported")
            }
            setState(STT_State.ready)
        }).catch(err => {
            console.error(err)
        })

        return () => {
            if (recognition)
                recognition.abort()
        }
    }, [])
    
    if (recognition)
        recognition.lang=ctx.locale

    let theme = useTheme()
    let classes = useStyle(theme)

    // handling when user click on icon to start or stop speech recognition
    let internalHandling = (e: React.MouseEvent) => {
        let prevState = state
        switch (state) {
            case STT_State.undefined:
                console.log("SpeechRecognition is not available")
                if (onStateChange) {
                    onStateChange(STT_State.ready, prevState)
                }
                break
            case STT_State.ready:
                console.log("Start recognition")
                recognition.start()
                recognition.onspeechend = () => {
                    console.log("End of speech detected")
                    // recognition.stop()
                    setState(STT_State.ready)

                    // User may already force stop but speech recognition may be not yet stop
                    if (onStateChange) {
                        onStateChange(STT_State.ready, STT_State.start)
                    }
                }

                if (onData) {
                    recognition.onresult = (e: any) => {
                        onData(e.results[0][0].transcript)
                    }
                }
                setState(STT_State.start)
                if (onStateChange) {
                    onStateChange(STT_State.start, prevState)
                }
                break
            case STT_State.start:
                console.log("Aborting recognition")
                recognition.abort()
                // we don't need to fire onStateChange
                // it's already handle by onspeechend hook at speechrecognition object
                break
            default:
                console.error("Unknown SpeechRecognition state")
        }
        if (onClick)
            onClick(e, prevState)
    }

    return (
        <>
            {state === STT_State.ready &&
                <Mic onClick={internalHandling} className={classes.root} fontSize={"large"}/>
            }
            {state === STT_State.start &&
                <Dot onClick={internalHandling}/>
            }
        </>
    )
}
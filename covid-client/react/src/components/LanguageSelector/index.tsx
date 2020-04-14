import React, {useContext} from "react"
import {FormControl, InputLabel, NativeSelect, ThemeProvider, useTheme} from "@material-ui/core"

import Context from "../../Context"

const supportedLocale: Record<string, string> = {
    "Thai": "th-TH",
    "US": "en-US"
}

export default function LanguageSelector({style}: {style?: React.CSSProperties}) {
    const theme = useTheme()
    const ctx = useContext(Context)
    
    return (
        <ThemeProvider theme={theme}>
            <FormControl style={{...style, width: "10vw"}}>
                <InputLabel id="langid" style={{color: theme.palette.text.primary}}>Language</InputLabel>
                <NativeSelect 
                    inputProps={{
                        name: 'langid',
                        id: 'langid',
                    }} 
                    value={ctx.locale} 
                    onChange={(e)=> {ctx.setLocale(e.target.value as string)}}
                >
                    {Object.keys(supportedLocale).map(k=>(
                        <option key={k} value={supportedLocale[k]}>{k}</option>
                    ))}
                </NativeSelect>
            </FormControl>
        </ThemeProvider>
    )
}
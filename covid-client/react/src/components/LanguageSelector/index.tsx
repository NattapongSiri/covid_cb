import React, {useContext} from "react"
import {FormControl, InputLabel, NativeSelect, ThemeProvider, useTheme} from "@material-ui/core"
import Context from "../../Context"

const supportedLocale: Record<string, string> = {
    "Arabic": "ar",
    "Chinese": "zh",
    "Dutch": "nl",
    "French": "fr",
    "German": "de",
    "Japanese": "ja",
    "Hebrew": "he",
    "Hindi": "hi",
    "Korea": "ko",
    "Portuguese": "pt",
    "Russian": "ru",
    "Spanish": "es",
    "Taiwan": "zh-TW",
    "Thai": "th",
    "US": "en"
}

export default function LanguageSelector({style}: {style?: React.CSSProperties}) {
    const theme = useTheme()
    const ctx = useContext(Context)
    
    return (
        <ThemeProvider theme={theme}>
            <FormControl style={{...style, width: "10vw"}}>
                <InputLabel id="langid" style={{color: theme.palette.text.primary}}>Language</InputLabel>
                {/* <Autocomplete options={{supportedLocale}} getOptionLabel={(option: Record<string, string>) => option[0]} renderInput={(params: any) => <TextField {...params} variant="outlined" /> }/> */}
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
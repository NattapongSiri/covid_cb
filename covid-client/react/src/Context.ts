import React from 'react'

export let defaultLocale = navigator.language?navigator.language:"en"

export default React.createContext({
    locale: defaultLocale, 
    light: false, 
    toggleLight: () => {}, 
    setLocale: (l: string)=>{}})
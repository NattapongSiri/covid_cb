import React from 'react'

export let defaultLocale = navigator.language?navigator.language:"en_US"

export default React.createContext({
    locale: defaultLocale, 
    light: false, 
    toggleLight: () => {}, 
    setLocale: (l: string)=>{}})
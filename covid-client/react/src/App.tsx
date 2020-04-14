import React, {useState} from 'react';
import "./App.css"
import ChatPage from './pages/ChatPage'
import {createMuiTheme, ThemeProvider} from "@material-ui/core/styles"
import Context, {defaultLocale} from './Context'

const darkTheme = createMuiTheme({
  palette: {
    background: {
      default: "#000"
    },
    text: {
      primary: "#FFF",
      secondary: "#FFF"
    },
    type: "dark"
  },
});

const lightTheme = createMuiTheme({
  palette: {
    background: {
      paper: "#FFE"
    },
    type: "light"
  }
})

function App() {
  const savedLocale = localStorage.getItem("locale")
  let [locale, setLocale] = useState(savedLocale?savedLocale:defaultLocale)
  let [light, setLight] = useState(localStorage.getItem("light") === "true")

  return (
    <div className="App">
      <header className="App-header">
        <Context.Provider 
          value={{
            locale, 
            light, 
            toggleLight: () => {
              localStorage.setItem("light", (!light).toString())
              setLight(!light)
            }, 
            setLocale: (l:string)=>{
              localStorage.setItem("locale", l)
              setLocale(l)
            }
          }}
        >
          <ThemeProvider theme={light?lightTheme:darkTheme}>
            <ChatPage />
          </ThemeProvider>
        </Context.Provider>
      </header>
    </div>
  );
}

export default App;

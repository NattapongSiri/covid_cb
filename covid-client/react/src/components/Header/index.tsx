import React, {useContext} from 'react'
import {Container, Switch} from '@material-ui/core'
import {SwitchClassKey, SwitchProps} from '@material-ui/core/Switch'
import {Theme} from '@material-ui/core/styles'
import {makeStyles, useTheme, withStyles} from '@material-ui/styles'
import WbIncandescentIcon from '@material-ui/icons/WbIncandescent'

import dark_logo from './watsondark.gif'
import light_logo from './watsonlight.gif'

import useStyle from './Header-style'
import Context from '../../Context'
import LanguageSelector from '../LanguageSelector'

const useSwitchClass = makeStyles({
    root: {
        borderRadius: "10vmin",
        backgroundColor: "#444",
        color: "#CCC",
        position: "absolute",
        top: "4vh",
        right: "15vw"
    },
})

interface Props extends SwitchProps {
    classes: Partial<Record<SwitchClassKey, string>>;
}

const LightSwitch = withStyles({
    root: {
        
    },
    switchBase: {
        '&$checked+ $track': {
            backgroundColor: '#888',
            opacity: 1,
        }
    },
    track: {
        backgroundColor: "#888",
        opacity: 1
    },
    checked: {}
})(({classes, ...props}: Props)=> (
    <Switch {...props}  classes={{track: classes.track, switchBase: classes.switchBase, checked: classes.checked}}/>
))

LightSwitch.displayName = "LightSwitch"

export default function Header() {
    let theme = useTheme() as Theme
    let classes = useStyle(theme)
    let switchClass = useSwitchClass()
    let ctx = useContext(Context)

    return (
        <Container className={classes.root}>
            {theme.palette.type === "dark" &&
                <img src={dark_logo} alt="IBM Watson"/>
            }
            {theme.palette.type !== "dark" &&
                <img src={light_logo} alt="IBM Watson"/>
            }
            <LightSwitch 
                className={switchClass.root}
                checked={ctx.light} 
                onChange={() => {
                    if (ctx.toggleLight) {
                        ctx.toggleLight()
                    }
                }}
                checkedIcon={<WbIncandescentIcon style={{color:"#FFF"}}/>} 
                icon={<WbIncandescentIcon style={{color:"#000"}} />}
            />
            <LanguageSelector style={{position: "absolute", top: "3vh", right: "2vw"}} />
        </Container>
    )
}
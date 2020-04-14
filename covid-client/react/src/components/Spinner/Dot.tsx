import React from 'react'
import {Theme} from '@material-ui/core/styles'
import {makeStyles, useTheme} from '@material-ui/styles'

const useStyle = makeStyles((theme: Theme) => ({
    root: {
        display: "inline-block",
        position: "relative",
        width: "10vw",
        height: "10vw",
        "& div": {
            position: "absolute",
            top: "45%",
            width: "15%",
            height: "15%",
            borderRadius: "50%",
            background: theme.palette.text.primary,
            animationTimingFunction: "cubic-bezier(0, 1, 1, 0)",
            "&:nth-child(1)": {
                left: "10%",
                animation: '$ldsEllipsis1 0.6s infinite'
            },
            "&:nth-child(2)": {
                left: "10%",
                animation: '$ldsEllipsis2 0.6s infinite'
            },
            "&:nth-child(3)": {
                left: "40%",
                animation: '$ldsEllipsis2 0.6s infinite'
            },
            "&:nth-child(4)": {
                left: "70%",
                animation: '$ldsEllipsis3 0.6s infinite'
            }
        }
    },
    "@keyframes ldsEllipsis1": {
        "0%": {
            transform: "scale(0)"
        },
        "100%": {
            transform: "scale(1)"
        }
    },
    "@keyframes ldsEllipsis3": {
        "0%": {
            transform: "scale(1)"
        },
        "100%": {
            transform: "scale(0)"
        }
    },
    "@keyframes ldsEllipsis2": {
        "0%": {
            transform: "translate(0, 0)"
        },
        "100%": {
            transform: "translate(3vw, 0)"
        }
    },
}))


export default function Dot({onClick}: {onClick?: (e: React.MouseEvent) => void}) {
    let theme = useTheme() as Theme
    let classes = useStyle(theme)
    return (
        <div onClick={onClick} className={classes.root}>
            <div />
            <div />
            <div />
            <div />
        </div>
    )
}
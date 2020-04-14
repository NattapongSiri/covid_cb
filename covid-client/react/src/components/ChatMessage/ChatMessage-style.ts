import {Theme} from "@material-ui/core/styles"
import { makeStyles } from "@material-ui/styles"

export const BubbleClass = makeStyles((theme: Theme) =>({
    self: {
        maxWidth: "75vw",
        width: "max-content",
        height: "max-content",
        display: "inline-block",
        color: theme.palette.text.primary,
        backgroundColor: theme.palette.background.paper,
        borderRadius: "4px",
        boxShadow: "2px 8px 5px #000",
        position: "relative",
        margin: "0 12px 0 0",
        zIndex: 10
    },
    other: {
        maxWidth: "75vw",
        width: "max-content",
        height: "max-content",
        display: "inline-block",
        color: theme.palette.text.primary,
        backgroundColor: theme.palette.background.paper,
        borderRadius: "4px",
        boxShadow: "2px 8px 5px #000",
        position: "relative",
        margin: "0 0 0 10px",
        zIndex: 10
    }
}))

export const BubbleArrowClass = makeStyles((theme: Theme) => ({
    self: {
        position: "absolute",
        width: 0,
        height: 0,
        right: "-2px",
        bottom: "40px",
        left: "auto",
        zIndex: -1,
        "&:after": {
            content: '""',
            position: "absolute",
            border: "0 solid transparent",
            borderTop: `9px solid ${theme.palette.background.paper}`,
            borderRadius: "0 20px 0",
            width: "15px",
            height: "30px",
            transform: "rotate(45deg) scaleY(-1)"
        },
    }, 
    other: {
        position: "absolute",
        width: 0,
        bottom: "42px",
        left: "-16px",
        height: 0,
        zIndex: -1,
        "&:after": {
            content: '""',
            position: "absolute",
            border: "0 solid transparent",
            borderTop: `9px solid ${theme.palette.background.paper}`,
            borderRadius: "0 20px 0",
            width: "15px",
            height: "30px",
            transform: "rotate(145deg)"
        }
    }
}))

export const SenderClass = makeStyles({
    self: {
        fontWeight: 600,
        fontSize: "0.7rem",
        color: "#66ff99",
        textAlign: "right"
    },
    other: {
        fontWeight: 600,
        fontSize: "0.7rem",
        color: "#66ccff",
        textAlign: "left"
    }
})

export const MsgClass = makeStyles((theme:Theme) => ({
    self: {
        backgroundColor: theme.palette.background.default,
        display: "flex",
        flexWrap: "wrap",
        alignItems: "flex-end",
        justifyContent: "flex-end",
        margin: "1vw 0",
        position: "relative"
    },
    other: {
        backgroundColor: theme.palette.background.default,
        display: "flex",
        flexWrap: "wrap",
        alignItems: "flex-end",
        justifyContent: "flex-start",
        margin: "1vw 0",
        position: "relative"
    }
}))

export const FlexBreaker = makeStyles({
    root: {
        flexBasis: "100%"
    }
})
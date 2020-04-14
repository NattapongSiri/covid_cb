import {Theme} from '@material-ui/core/styles'
import {makeStyles} from "@material-ui/styles"

export default makeStyles((theme: Theme) => ({
    root: {
        display: "block",
        height: "12vh",
        "& > img": {
            height: "inherit",
            width: "auto",
            display: "inline",
            position: "absolute",
            top: "0",
            left: "50%",
            transform: "translateX(-50%)"
        }
    }
}))
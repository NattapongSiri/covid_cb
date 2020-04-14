import {Theme} from '@material-ui/core/styles'
import {makeStyles} from "@material-ui/styles"
export default makeStyles((theme: Theme) => ({
    root: {
        alignItem: "center",
        borderRadius: "50%",
        border: "1px solid grey",
        color: theme.palette.text.primary,
        cursor: "pointer",
        margin: "0 2vw",
        padding: "1vw"
    }
}))
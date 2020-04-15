import React, {useContext} from "react"
import { Avatar, Button, Card, CardActionArea, CardActions, CardContent, CardHeader, Divider, Typography } from "@material-ui/core"
import { Theme, ThemeProvider } from "@material-ui/core/styles"
import { makeStyles, useTheme } from "@material-ui/styles"
import watson from './watson-avatar.png'

import { Message, MessageState } from '../commons/Message'
import Context from '../../Context'

import { BubbleClass, BubbleArrowClass, MsgClass, SenderClass, FlexBreaker } from "./ChatMessage-style"

export default function ChatMessage({message, onChoose}: {message: Message, onChoose?: (msg: string) => void}) {
    const self = message.type === "self"
    const theme = useTheme<Theme>()
    const bb = BubbleClass(theme)
    const after = BubbleArrowClass(theme)
    const sender = SenderClass()
    const msg = MsgClass(theme)
    const breakFlex = FlexBreaker()
    const nopadding = makeStyles({root:{padding: "1vh 2vw"}})()
    const ctx = useContext(Context)
    const dateTimeFormatter = new Intl.DateTimeFormat(ctx.locale, {
        year: 'numeric', month: 'numeric', day: 'numeric',
        hour: 'numeric', minute: 'numeric', second: 'numeric'
    })

    return (
        <ThemeProvider theme={theme}>
            <div className={self?msg.self:msg.other}>
                {!self && <Avatar style={{display:"inline-block"}} src={watson}/>}
                <div className={self?bb.self:bb.other}>
                    <Card elevation={0}>
                        <CardHeader classes={{root: nopadding.root, subheader: self?sender.self:sender.other}} subheader={self?"You":"Watson"}  titleTypographyProps={{variant:'body2' }} />
                        <Divider />
                        <CardContent style={{
                                padding: "8px 55px 8px 14px",
                                margin: 0
                            }}>
                            <Typography style={{overflow: "auto"}}> 
                                <span style={{textAlign: "left"}} dangerouslySetInnerHTML={{__html: message.message}}/>
                            </Typography> 
                        </CardContent>
                        {message.suggestions && 
                            <>
                                <Divider />
                                <CardContent>
                                    {message.suggestions.map(s => (
                                        <Button
                                            key={s.uuid} 
                                            style={{margin: "2vmin"}} 
                                            variant={"outlined"} 
                                            onClick={() => {
                                                if (onChoose)
                                                    onChoose(s.label)
                                            }}
                                        >{
                                        s.label
                                        }</Button>
                                    ))}
                                </CardContent>
                            </>
                        }
                        {message.reference && 
                            <>
                                <Divider />
                                <CardActions>
                                    <Button variant="outlined" onClick={() => {window.open(message.reference)}}>Click to view original page</Button>
                                </CardActions>
                            </>
                        }
                        {message.previewUrl &&
                            <>
                                <Divider />
                                <CardActionArea>
                                    {
                                    <iframe title={message.message} style={{width: "75vw", height: "75vh"}} src={message.previewUrl.toString()}/>
                                    }
                                    {/*
                                    <object data={message.previewUrl.toString()} style={{width: "75vw", height: "75vh"}} type="text/html">Preview is not supported</object>
                                    */}
                                </CardActionArea>
                            </>
                        }
                    </Card>
                    <div className={self?after.self:after.other} 
                    // Pointy tale of chat bubble
                    />
                </div>
                {self && <Avatar style={{display:"inline-block"}} />}
                <div className={breakFlex.root}/>
                {message.state === MessageState.succeed &&
                    <Typography color={"textSecondary"} variant={"body2"} display={"block"} style={{margin: self?"3px 55px 1vh 0":"3px 0 1vh 55px"}}>
                        {dateTimeFormatter.format(message.timestamp)}
                    </Typography>
                }
                {message.state === MessageState.sending &&
                    <Typography variant={"body2"} display={"block"} style={{margin: self?"3px 55px 1vh 0":"3px 0 1vh 55px", color: theme.palette.type==="dark"?theme.palette.warning.dark:theme.palette.warning.light}}>
                        Sending
                    </Typography>
                }
                {message.state === MessageState.failed &&
                    <Typography variant={"body2"} display={"block"} style={{margin: self?"3px 55px 1vh 0":"3px 0 1vh 55px", color: theme.palette.type==="dark"?theme.palette.error.dark:theme.palette.error.light}}>
                        Failed
                    </Typography>
                }
            </div>
        </ThemeProvider>
    )
}
const {IamAuthenticator} = require("ibm-watson/auth")
const AssistantV2 = require('ibm-watson/assistant/v2')
require("dotenv").config()

async function main({context, message, sessionId, sourceLang="en", userid="guest"}) {
    let {apikey, assistantId} = process.env
    let authenticator = new IamAuthenticator({
        apikey: apikey
    })
    let assistant = new AssistantV2({
        authenticator,
        serviceUrl: "https://api.us-south.assistant.watson.cloud.ibm.com/instances/081b5b03-742c-47bc-abd7-b52eb636b909",
        version: "2020-04-01"
    })

    let response = await assistant.message({
        assistantId: assistantId,
        sessionId: sessionId,
        input: {
            message_type: "text",
            options: {
                alternate_intents: true,
                return_context: true,
            },
            text: message
        },
        context: {
            global: {
              system: {
                user_id: userid
              }
            },
            skills: {
                'main skill': {
                  user_defined: {
                      ...context
                  }
                }
            }
        }
    })

    return {
        sourceLang,
        ...response
    }
}

exports.main=main
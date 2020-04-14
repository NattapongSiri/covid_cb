const {IamAuthenticator} = require("ibm-watson/auth")
const AssistantV2 = require('ibm-watson/assistant/v2')
require("dotenv").config()

function main() {
    let {apikey, assistantId} = process.env
    let authenticator = new IamAuthenticator({
        apikey: apikey
    })
    let assistant = new AssistantV2({
        authenticator,
        serviceUrl: "https://api.us-south.assistant.watson.cloud.ibm.com/instances/081b5b03-742c-47bc-abd7-b52eb636b909",
        version: "2020-04-01"
    })

    return assistant.createSession({assistantId: assistantId})
}

exports.main=main
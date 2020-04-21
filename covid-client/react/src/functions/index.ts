const {
    REACT_APP_APIKEY: APIKEY, 
    REACT_APP_CREATE_SESSION_ENDPOINT: CREATE_SESSION_ENDPOINT, 
    REACT_APP_SEND_MSG_ENDPOINT: SEND_MSG_ENDPOINT
}=process.env

export interface Intent {
    intent: string
    confidence: number
}

export interface Entity {
    entity: string
    location: number[]
    value: string
    confidence?: number
    metadata?: any
    groups?: {
        group: string,
        integer: number[]
    },
    interpretation?: {
        calendar_type?: string
        datetime_link?: string
        festival?: string
        granularity?: string
        range_link?: string
        range_modifier?: string
        relative_day?: number
        relative_month?: number
        relative_week?: number
        relative_weekend?: number
        relative_year?: number
        specific_day?: number
        specific_day_of_week?: string
        specific_month?: number
        specific_quarter?: number
        specific_year?: number
        numeric_value?: number
        subtype?: string
        part_of_day?: string
        relative_hour?: number
        relative_minute?: number
        relative_second?: number
        specific_hour?: number
        specific_minute?: number
        specific_second?: number
        timezone?: string
    },
    alternatives?: [
        {
            value: string,
            confidence: number
        }
    ],
    role?: "date_from" | "date_to" | "number_from" | "number_to" | "time_from" | "time_to"
}

export interface Message {
    message: string
    sourceLang: string
    targetLang?: "en" | "en-US" // It currently support only English

    sessionId?: string
    context?: any
    userid?: string
    intents?: Intent[]
    entities?: Entity[]
}

export async function createWASession() {
    if (CREATE_SESSION_ENDPOINT && APIKEY) {
        let response = await fetch(CREATE_SESSION_ENDPOINT, {
            method: "POST",
            cache: 'no-cache',
            headers: {
                "X-IBM-Client-Id": APIKEY
            }
        })
        let data = await response.json()
        return data
    } else {
        console.error("Undefined messaging endpoint or APIKEY is invalid")
    }
}

export async function sendMessage(message: Message) {
    message.targetLang = "en"
    if (SEND_MSG_ENDPOINT && APIKEY) {
        let response = await fetch(SEND_MSG_ENDPOINT, {
            method: "POST",
            cache: 'no-cache',
            headers: {
                'Content-Type': 'application/json',
                "X-IBM-Client-Id": APIKEY
            },
            body: JSON.stringify(message)
        })

        let data = await response.json()

        return data
    } else {
        console.error("Undefined messaging endpoint or APIKEY is invalid")
    }
}

export async function closeWASession(sessionId: string) {
    if (CREATE_SESSION_ENDPOINT && APIKEY) {
        let response = await fetch(CREATE_SESSION_ENDPOINT, {
            method: "POST",
            cache: 'no-cache',
            headers: {
                "X-IBM-Client-Id": APIKEY
            },
            body: `{"sessionId":${sessionId}}`
        })
        let data = await response.json()
        return data
    } else {
        console.error("Undefined messaging endpoint or APIKEY is invalid")
    }
}
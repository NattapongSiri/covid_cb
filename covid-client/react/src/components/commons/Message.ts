export interface Suggestion {
    dialog_node: string
    internal: {
        propensity: number
    }
    label: string
    output: {
        generic: [{
            label: string
            value: {
                input: {
                    text: string
                }
            }
        }]
    }
    value: {
        input: {
            entities: []
            intents: [{
                intent: string
                confidence: number
            }]
        },
        suggestionId: string
    }
}

export interface Message {
    uuid: string
    message: string
    type: "self" | string
    timestamp: Date
    reference?: string // for example: url of source of this message
    previewUrl?: URL // message associate with link to be preview

    suggestions?: [{
        uuid: string
        label: string
        intents?: [{
            intent: string
            confidence: number
        }]
    }]

    results?: [{
        uuid: string
        title: string
        highlight: string
        url?: string
    }]
}
# input-translate
It take input message as well as defined sourceLang parameter and targetLang parameter. If source lang and target lang is not equals then it call Watson Language Translator then replace user message with translated message.
This functions take following parameters:
1. context - JSON object that will be taken into Watson Assistant "main skill" context.
1. sessionId - Watson Assistant Session Id, if any
1. message - Source user message
1. sourceLang - Language of given message
1. targetLang - Expected language where Watson Assistant can understand
This function return JSON object with following parameters:
1. context - JSON object from input parameters
1. sessionId - Watson Assistant Session Id from input parameters
1. message - Translated user message
1. sourceLang - Original message language
1. targetLang - Language of current message
It's design to be composable with `msg-wa` sub-project so that input/output of this functions can be replaced or forward by `msg-wa` function.
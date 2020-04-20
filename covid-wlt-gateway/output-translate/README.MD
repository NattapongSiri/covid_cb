# output-translate
It's cloud function that take response of Watson Assistant and translate it back to given `sourceLang` parameter.
It translate only for `option`, `suggestion`, and `text` type of response.
It directly manipulate the respons object so the structure of response remain the same.

This functions take following parameters:
1. status - The HTTP response code. It'll try to translate only if it is 200
1. result - Watson Assistant response object
1. sourceLang - The expected language by user
1. targetLang - The language where Watson Assistant understand

This function return following JSON object:
1. status - The HTTP response code from Watson Assistant
1. result - The Watson Assistant compatible result object.
It may throw some error from Watson Language Translator as this function doesn't catch translation error.
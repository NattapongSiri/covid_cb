require('dotenv').config()
const LanguageTranslatorV3 = require('ibm-watson/language-translator/v3');
const { IamAuthenticator } = require('ibm-watson/auth');

const languageTranslator = new LanguageTranslatorV3({
  version: '2018-05-01',
  authenticator: new IamAuthenticator({
    apikey: process.env.APIKEY,
  }),
  url: process.env.ENDPOINT_URL,
});

async function main({context, sessionId, message : text, sourceLang, targetLang}) {
  if (sourceLang != targetLang) {
    const translateParams = {
      text,
      source: sourceLang,
      target: targetLang
    };

    let response = await languageTranslator.translate(translateParams)
    if (response.status === 200) {
        return {
            context,
            sessionId,
            sourceLang,
            message: response.result.translations[0].translation,
        }
    } else {
        throw new Error(response.statusText)
    }
  } else {
    // simply forward request if source and target is the same language
    return {
        context,
        sessionId,
        sourceLang,
        message: text,
    }
  }
}

exports.main = main
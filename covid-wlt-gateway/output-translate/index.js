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

async function main({status, result, sourceLang, targetLang = "en"}) {
  if (status == 200) {
    if (result.output.generic.length > 0 && sourceLang != targetLang) {
      // need to translate something in response

      // create a map that will be use to put translated back into result object
      let translate = [] // response object index to be translate
      let sub_translate = [] // size of sub-response to be translate such as options/suggestions
      let text = [] // raw text to be translate

      for (let i = 0; i < result.output.generic.length; i++) {
        if (result.output.generic[i].response_type === "text") {
          let r = result.output.generic[i].text.trim()
          // skip translate if message begin with link
          if (!r.startsWith("<a ") && !r.startsWith("http")) {
            translate.push(i)
            sub_translate.push(0)
            text.push(result.output.generic[i].text)
          }
        } else if (result.output.generic[i].response_type === "suggestion") {
          translate.push(i)
          text.push(result.output.generic[i].title)
          let j = 1
          for (; j < result.output.generic[i].suggestions.length; j++) {
            text.push(result.output.generic[i].suggestions[j].label)
          }
          sub_translate.push(j)
        } else if (result.output.generic[i].response_type === "option") {
          translate.push(i)
          text.push(result.output.generic[i].title)
          let j = 1
          for (; j < result.output.generic[i].options.length; j++) {
            text.push(result.output.generic[i].options[j].label)
          }
          sub_translate.push(j)
        } 
      }
      const translateParams = {
        text: text,
        source: targetLang,
        target: sourceLang
      };
    
      let response = await languageTranslator.translate(translateParams)
      if (response.status === 200) {
        // map translation back to WA response
        let i = 0
        let j = 0
        response.result.translations.forEach((({translation}) => {
          // no more possible sub-translation to map 
          if (j > sub_translate[i]) {
            i++
            j = 0
          }

          if (result.output.generic[translate[i]].response_type === "text") {
            result.output.generic[translate[i]].text = translation

            // it's single text response translation
            i++
            j = 0
          } else if (result.output.generic[translate[i]].response_type === "suggestion") {
            if (j != 0) {
              result.output.generic[translate[i]].suggestions[j - 1].label = translation
            } else {
              result.output.generic[translate[i]].title = translation
            }
            
            // possibly have more suggestions
            j++
          } else if (result.output.generic[translate[i]].response_type=== "option") {
            if (j != 0)
              result.output.generic[translate[i]].options[j - 1].label = translation
            else
              result.output.generic[translate[i]].title = translation
            
              // possible have more options
            j++
          }
        }))

        return {
          status,
          result
        }
      } else {
        // WLT returne error
          throw new Error(response.statusText)
      }
    } else {
      // nothing return or source and target lang is the same, just forward response back to user
      return {
        status,
        result
      }
    }
  } else {
    throw new Error("Error was forwarded by previous operation")
  }
}

exports.main = main
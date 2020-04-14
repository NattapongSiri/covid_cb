# IBM Cloud Functions that act as API gateway
This is a bunch of cloud functions sub-project.

It's mandatory to create `.env` file in each sub-project root directory with following lines inside it.
```
apikey=<Watson Assistant IAM api key>
assistantId=<Watson Assistant's assistant ID>
```

It contains following key sub-projects

# Common requirements
1. ibmcloud cli
1. ibmcloud cli plugin for cloud functions
1. Watson Assistant instance information
    1. IAM api key 
    1. Watson Assistant's assitant id 

# How to build
1. `ibmcloud login` or `ibmcloud login --sso` if you account have SSO associated with IBM cloud
1. Create `.env` file on root of sub-project and put following line in the file
    1. apikey=<YOUR_API_WATSON_ASSISTANT_API_KEY>
    1. assistantId=<ASSISTANT_ID>
1. Zip entire sub-project directory. Do not zip the root of project. See below on expected structure inside zip file.
1. Set your ibmcloud function cli to use your name space by issue this command `ibmcloud fn property set --namespace <namespace_name>` where <namespace_name> is your cloud functions' namespace
1. Do one of following
    1. Create your cloud function by issue this command `ibmcloud fn action create <your_function_name> <your_zip_file> --kind nodejs:default -m 128 -t 15000`. Replace `<your_function_name>` with the name to be used on IBM Cloud Functions and `<your_zip_file>` with the zip file in prior step. The functions name shall be unique
    1. Create your cloud function by issue this command `ibmcloud fn action update <your_function_name> <your_zip_file> --kind nodejs:default -m 128 -t 15000`. Replace `<your_function_name>` with the name of existing IBM Cloud Functions and `<your_zip_file>` with the zip file in prior step. 

# Zipped content structure
The inside zip file shall have following structure:
```
node_modules
.env
index.js
package.json
README.MD
```
It shall not have root directory of project inside zip file.

# Sub-projects
Following is available sub-projects that will be needed to properly operate this chatbot
## msg-wa
It is cloud functions that redirect user input to Watson Assistant
## start-wa
It is cloud function that create Watson Assistant session and return session id to caller
# stop-wa
It is cloud function that destroy Watson Assistant session from given session id
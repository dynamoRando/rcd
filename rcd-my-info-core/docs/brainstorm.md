# Overview

We need software to allow us to manage multiple RCD instances on behalf of customers. An RCD instance is provisioned via a container.

Expecations:
    - rcd-my-info-core will be able to leverage both docker or k8s, depending on configuration
    - rcd-my-info will be the web page front-end to allow users to admin their rcd instance
    - rcd-admin will be the local instance for the users to admin. if possible, we don't want to 
    duplicate functionality here between rcd-admin and rcd-my-info. rcd-my-info is the SaaS that
    allows multiple users to create data accounts (implemented via rcd). rcd-admin is the software
    that allows users to actually manage their own instance of rcd.

Setup:
    - Verify that the container "rcd:latest" is available
    - Provision a container with a unique id tied to a registered user of my-info
    - Configure that container with a un and pw of the registered users' choosing
        - If possible, either shell an instance or allow a user to be able to connect to 
        that provision'd instance of rcd-admin on behalf of the user
        - If not possible, then need to break out rcd-admin into a reusable layer
        that rcd-my-info can leverage to admin an rcd instance on behalf of a user 


References: 

 - https://stackoverflow.com/questions/63128587/how-to-connect-to-a-docker-container-using-a-domain-name
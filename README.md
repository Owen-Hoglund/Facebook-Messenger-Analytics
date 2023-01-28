# Facebook-Messenger-Analytics
Small project for gathering data on your own facebook messenger activity. 
For obvious security reasons running this code on your own machine will take some setup.
1. Download your messenger data from facebook - [tutorial here](https://www.facebook.com/help/messenger-app/713635396288741). This is likely to be multiple files.
2. Once you have downloaded all of your files, you need to add a `.env` file to the projects root directory. 
3. Now you need to find the conversation you are interested in getting analytics out of. This might take some time as the downloads from facebook are a bit labyrinthine. 
You will know you are in the right place if you start finding json files. Once you have found the json files associated with the conversation you are interested in, copy the *path* to the directory that contains those files.
4. Now you need to add "TEST_DIRECTORY='{path from step 4}'" to the .env file you created.
5. Run the program from the command line with cargo run (this assumes you have Rust on your machine) 

I do plan on automating as much as I can from the steps above, but again, due to obvious security concerns regarding your personal messages, I can't do everything for the user. 

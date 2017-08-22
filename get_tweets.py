import tweepy
from tweepy import OAuthHandler
import tokens
import json


auth = OAuthHandler(tokens.consumer_key, tokens.consumer_secret)
auth.set_access_token(tokens.access_token, tokens.access_secret)

api = tweepy.API(auth)

tweets=[]
counter=0
for status in tweepy.Cursor(api.user_timeline, id='realDonaldTrump').items():
    # Process a single status
    tweets.extend([{"text": status.text}])

print(json.dumps(tweets))

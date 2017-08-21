import tweepy
from tweepy import OAuthHandler
import tokens


auth = OAuthHandler(tokens.consumer_key, tokens.consumer_secret)
auth.set_access_token(tokens.access_token, tokens.access_secret)

api = tweepy.API(auth)

print('[')
counter=0
for status in tweepy.Cursor(api.user_timeline, id='realDonaldTrump').items():
    # Process a single status
    print((',' if counter > 0 else ''))
    print(status._json)
    counter +=1
print(']')

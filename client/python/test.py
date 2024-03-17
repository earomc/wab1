import requests
import json

# Define your GraphQL query
query = """
{
  products(priceFilter: { min: 10, max: 100 }) {
    id
    name
    price
  }
}
"""

# Define the URL of your GraphQL server endpoint
url = 'http://localhost:8080/graphql'

# Define the request headers
headers = {'Content-Type': 'application/json'}

# Define the request payload as JSON
data = {'query': query}

# Send the HTTP POST request
response = requests.post(url, headers=headers, json=data)

# Check if the request was successful
if response.status_code == 200:
    # Parse the JSON response
    json_data = response.json()
    # Print the response data
    print(json.dumps(json_data, indent=2))
else:
    print('Error:', response.text)
import json

def lambda_handler(event, context):
    msg = json.loads(event['Records'][0]['body'])
    num = msg['num']
    print(fib_recur(num))


def fib_recur(num):
    return num if num <= 1 else fib_recur(num - 1) + fib_recur(num - 2)

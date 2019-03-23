import sys
import random
import requests
import multiprocessing


threadCount = 4
wordIdMin = 1
wordIdMax = 55
wordChoiceMin = 5
wordChoiceMax = 6
boyPercent = 0.5
fullNamePercent = 0.66
middleInitialPercent = 0.10
fakeNamerPercent = 0.99
sendAmount = 10_000
wordsOveride = [] # [56, 57, 58, 59, 60, 61]
# TargetNumber = 16_000
# CurrentNumber = 15_574 

def asReqLoop(pName, boys, girls, last, userAgents):

    for i in range(sendAmount):
        print(f"Done {i} times from thread {pName}")
        name =  randomName(boys, girls, last)
        userAgent = randomAgent(userAgents)
        words = wordsOveride if len(wordsOveride) > 0 else randomWords()
        dnt = "0" if random.random() > 0.5 else "1"
        upgrade = "0" if random.random() > 0.5 else "1"
        
        headers = {
            'User-Agent': userAgent,
            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
            'Accept-Language': 'en-US,en;q=0.5',
            'Content-Type': 'application/x-www-form-urlencoded',
            'DNT': dnt,
            'Connection': 'keep-alive',
            'Upgrade-Insecure-Requests': upgrade,
        }

        data = {
        'voter': name,
        'name': 'xx',
        'words': words
        }

        response = requests.post('https://kevan.org/johari.cgi', headers=headers, data=data)
        if response.status_code != 200:
            print(f"thread {pName} received a non-200 code, stopping thread")
            return
    print(f"thread {pName} has finished")


def randomWords(encode=False):
    amount = random.randint(wordChoiceMin, wordChoiceMax)
    nums = []
    ids = [x for x in range(wordIdMin, wordIdMax+1)]
    random.shuffle(ids)
    for _ in range(amount):
        word = ids.pop()
        nums.append(word)
    join = '"%"2C' if encode else ','
    s = join.join([str(x) for x in nums])
    return s

def randomAgent(uas):
    agent = random.sample(uas, 1)[0]
    return agent

def readNames():
    with open("first_names.txt", 'r') as f:
        firstNames = [x.strip() for x in f.readlines()]
        ll = len(firstNames)//2
        girls = firstNames[:ll]
        boys = firstNames[ll:]
    with open("last_names.txt", 'r') as f:
        lastNames = [x.strip() for x in f.readlines()]
    return boys, girls, lastNames

def readUA():
    uas = []
    with open("user_agents.txt", 'r') as f:
        uas = [x.strip() for x in f.readlines() if x]
    return uas

def randomName(boys, girls, last, encode=False):
    if random.random() > boyPercent:
        use = boys
    else:
        use = girls

    flen = len(use)
    flenrand = random.randint(0, flen-1)

    first = use[flenrand]
    r = random.random()
    if r < fullNamePercent:
        return first
    else:
        llen = len(last)
        llenrand = random.randint(0, llen-1)
        enc = "+" if encode else ' '
        name = f'{use[flenrand]}{enc}{last[llenrand]}'
        return name


def main():
    uas = readUA()
    boys, girls, last = readNames()

    threads = []
    for x in range(threadCount):
        threads.append(multiprocessing.Process(target=asReqLoop, args=(f"p{x}", boys, girls, last, uas, )))

    for t in threads:
        t.start()

    for t in threads:
        t.join()


    return 0

if __name__ == "__main__":
    sys.exit(main())
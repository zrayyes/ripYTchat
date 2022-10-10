import datetime
import re
import sys
from time import sleep
from typing import Dict

import requests


def get_value_from_body(value: str, body: str) -> str | None:
    result1 = re.search(f'"{value}":"(.*?)",', r.text)
    if result1:
        return result1.group(1)


def get_post_body(continuation: str) -> Dict:
    return {
        "context": {
            "client": {
                "userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/\
                93.0.4577.63 Safari/537.36,gzip(gfe)",
                "clientName": "WEB",
                "clientVersion": "2.20210909.07.00",
            }
        },
        "continuation": continuation,
    }


def get_messages_from_renderer(render: Dict) -> str:
    output = ""
    runs = live_chat_message_renderer["message"]["runs"]
    for item in runs:
        if "text" in item:
            output += item["text"]
        elif "emoji" in item:
            output += item["emoji"]["shortcuts"][0]
    return output


if len(sys.argv) < 2:
    print("Please add video id.")
    print("Example: python ripytc.py dPX0_IEXVRo")
    exit(1)

for video_id in sys.argv[1::]:
    try:
        video_url = f"https://www.youtube.com/watch?v={video_id}"

        s = requests.Session()

        r = s.get(video_url)
        continuation = ""
        api_key = ""

        try:
            continuation = get_value_from_body("continuation", r.text)
            api_key = get_value_from_body("INNERTUBE_API_KEY", r.text)
        except AttributeError:
            print("Video not found!")
            exit(1)

        live_chat_url = (
            f"https://www.youtube.com/youtubei/v1/live_chat/get_live_chat_replay?key={api_key}"
        )

        counter = 0
        with open(f"{video_id}.txt", "w", encoding="utf-8") as f:
            while continuation is not None:
                r = s.post(live_chat_url, json=get_post_body(continuation))

                r_json_body = r.json()
                actions = []
                try:
                    actions = r_json_body["continuationContents"]["liveChatContinuation"][
                        "actions"
                    ]
                except KeyError:
                    break
                for action in actions:
                    try:
                        live_chat_message_renderer = action["replayChatItemAction"]["actions"][
                            0
                        ]["addChatItemAction"]["item"]["liveChatTextMessageRenderer"]
                        author_name = live_chat_message_renderer["authorName"]["simpleText"]
                        message = get_messages_from_renderer(live_chat_message_renderer)
                        offset_time_msec = int(
                            action["replayChatItemAction"]["videoOffsetTimeMsec"]
                        )
                        offset_datetime = datetime.timedelta(milliseconds=offset_time_msec)
                        offset_hh_mm_ss = str(offset_datetime).split(".")[0]
                        out = f"[{offset_hh_mm_ss} | {offset_datetime.seconds}] {author_name} --- {message} \n"
                        f.write(out)
                        counter += 1
                    except KeyError:
                        continue

                try:
                    tmp_continuation = r_json_body["continuationContents"][
                        "liveChatContinuation"
                    ]["continuations"][0]["liveChatReplayContinuationData"]["continuation"]
                    continuation = tmp_continuation
                except KeyError:
                    continuation = None

                print(counter)
                sleep(0.25)
    except KeyboardInterrupt:
        break

print("Done!")

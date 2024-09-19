import requests
from bs4 import BeautifulSoup

url = "https://coolpc.com.tw/evaluate.php"
response = requests.get(url)

if response.status_code == 200:
    soup = BeautifulSoup(response.text, 'html.parser')

    select_elements = soup.find_all('select')

    for select in select_elements:
        print(f"選單名稱: {select.get('name')}")
        options = select.find_all('option')
        for option in options:
            print(f"選項: {option.text}")
else:
    print("無法取得網頁內容")

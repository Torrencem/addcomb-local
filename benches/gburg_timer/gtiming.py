from time import time
from selenium  import webdriver
from selenium.webdriver.common.keys import Keys

from selenium.webdriver.support.ui import WebDriverWait as wait
from selenium.webdriver.support import expected_conditions as EC

params = {
    "n": 45,
    "m": 5,
    "hspec": None,
    "hrange": (1, 3),
    "addonly": True,
    "repeat": True,
    "size_restr": (45, 45)
}


driver = webdriver.Firefox()
driver.get("http://www.addcomb.gettysburg.edu/sumsets.html")
assert "Sumset" in driver.title
gn = driver.find_element_by_id("n")
gn.clear()
gn.send_keys(str(params["n"]))

m = driver.find_element_by_id("m")
m.clear()
m.send_keys(str(params["m"]))

h = driver.find_element_by_id("h")
if params["hspec"]:
    h.clear()
    h.send_keys(str(params["hspec"]))
else:
    driver.find_elements_by_name("h_rb")[1].click()
    driver.find_element_by_name("h_min").send_keys(str(params["hrange"][0]))

    driver.find_element_by_name("h_max").send_keys(str(params["hrange"][1]))

if not params["addonly"]:
    driver.find_elements_by_name("sub_rb")[1].click()
if not params["repeat"]:
    driver.find_elements_by_name("repeat_rb")[1].click()

driver.find_elements_by_name("size_rb")[1].click()

driver.find_element_by_id("s_min").send_keys(str(params["size_restr"][0]))
driver.find_element_by_id("s_max").send_keys(str(params["size_restr"][1]))

driver.find_elements_by_name("all_rb")[1].click()

start = time()

driver.find_element_by_xpath("/html/body/center/form/table/tbody/tr[8]/td/center/input").click()

# find new window
for handle in driver.window_handles:
    driver.switch_to_window(handle)

wait(driver, 150).until(EC.url_contains("http://www.addcomb.gettysburg.edu/cgi-bin/addcomb/sumsets.cgi"))
end = time()

print("time: " + str(end - start))

# driver.close()
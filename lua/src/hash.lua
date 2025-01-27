local md5 = require("md5")

local input_string = "Hello, World!"
local hash_value = md5.sumhexa(input_string) -- Calculates the MD5 hash in hexadecimal

print("MD5 Hash:", hash_value)
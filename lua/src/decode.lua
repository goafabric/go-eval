-- exemplary section if we would want to use the token endpoint, that should return token + tenantId
local cjson = require("cjson.safe")
local data, decode_err = cjson.decode(res.body)
ngx.req.set_header("Authentication", "Bearer " .. data.token)
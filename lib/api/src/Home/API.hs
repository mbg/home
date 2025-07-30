
-- | Implements the API.
module Home.API (
    API,
    runApiServer
) where

--------------------------------------------------------------------------------

import Servant

import Network.Wai.Handler.Warp ( run )

--------------------------------------------------------------------------------

-- | The API as a type.
type API = Get '[JSON] Int

-- | A term-level proxy for `API`.
api :: Proxy API
api = Proxy

-- | An implementation of `API`.
server :: Server API
server = pure 5

-- | A WAI `Application` for `server`.
app :: Application
app = serve api server

-- | `runApiServer` starts the API on port 8080.
runApiServer :: IO ()
runApiServer = run 8080 app

--------------------------------------------------------------------------------


-- | Implements the API.
module Home.API (
    API,
    runApiServer
) where

--------------------------------------------------------------------------------

import Control.Monad.IO.Class
import Control.Monad.Logger

import Data.Maybe
import Data.Text

import Servant

import Network.Wai.Handler.Warp ( run )

import Home.API.Config
import Home.Db

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

-- | `runApiServer` starts the API on the port given by @cfg@.
runApiServer :: ApiConfig Text -> IO ()
runApiServer cfg =
    runStdoutLoggingT $
    withDatabase (apiPostgres cfg) $ \dbPool -> do
        liftIO $ run port app
        where port = fromMaybe 8080 (apiPort cfg)

--------------------------------------------------------------------------------

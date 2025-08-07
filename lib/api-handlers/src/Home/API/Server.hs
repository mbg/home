-- | Implements the API server.
module Home.API.Server (
    API,
    runApiServer
) where

--------------------------------------------------------------------------------

import Control.Monad.Logger ( runStdoutLoggingT )
import Control.Monad.IO.Class

import Data.Maybe ( fromMaybe )
import Data.Text

import Servant

import Network.Wai.Handler.Warp ( run )

import Home.API
import Home.API.Config
import Home.API.Server.ApiError ( jsonErrorFormatters )
import Home.API.Server.Context
import Home.API.Server.Handler
import Home.API.Server.Finance
import Home.Db
import Home.Db.Schema ( migrateAll )

--------------------------------------------------------------------------------

-- | An implementation of `API`.
server :: ServerT API ApiHandler
server = pure 5 :<|> financeHandlers

-- | A custom Servant `Context` which installs custom error formatters.
servantContext :: Context '[ErrorFormatters]
servantContext = jsonErrorFormatters :. EmptyContext

-- | A WAI `Application` for `server`.
app :: ApiContext -> Application
app ctx =
    serveWithContext api servantContext $
    hoistServer api (fromApiHandler ctx) server

-- | `runApiServer` starts the API on the port given by @cfg@.
runApiServer :: ApiConfig Text -> IO ()
runApiServer cfg =
    runStdoutLoggingT $
    withDatabase (apiPostgres cfg) $ \dbPool -> do
        -- Run safe database migrations
        _ <- liftIO $ withPool dbPool $ runMigration migrateAll

        let ctx = MkApiContext dbPool
        liftIO $ run port (app ctx)
        where port = fromMaybe 8080 (apiPort cfg)

--------------------------------------------------------------------------------

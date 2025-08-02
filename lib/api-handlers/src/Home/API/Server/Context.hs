-- | Implements the `ApiContext` type that represents read-only state for API
-- handlers.
module Home.API.Server.Context (
    ApiContext(..)
) where

--------------------------------------------------------------------------------

import Home.Db ( DbPool )

--------------------------------------------------------------------------------

-- | Represents read-only state for API handlers.
data ApiContext = MkApiContext {
    -- | The resource pool for database connections.
    apiContextDbPool :: DbPool
}

--------------------------------------------------------------------------------

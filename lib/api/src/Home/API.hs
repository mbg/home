
-- | Implements the API.
module Home.API (
    API,
    api
) where

--------------------------------------------------------------------------------

import Servant

--------------------------------------------------------------------------------

-- | The API as a type.
type API
    = Get '[JSON] Int

-- | A term-level proxy for `API`.
api :: Proxy API
api = Proxy

--------------------------------------------------------------------------------

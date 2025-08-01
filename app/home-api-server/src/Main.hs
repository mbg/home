
-- | Contains `main`, which is the main entry point for the API server program.
module Main ( main ) where

--------------------------------------------------------------------------------

import Data.Text ( Text )

import System.Exit ( exitFailure )

import Home.Config.SecretSource
import Home.API (runApiServer)
import Home.API.Config

--------------------------------------------------------------------------------

-- | `readConfig` @path@ tries to read the APi server configuration from @path@
-- or fails by terminating the program.
readConfig :: FilePath -> IO (ApiConfig SecretSource)
readConfig fp = readApiConfig fp >>= \case
    Left err -> do
        putStrLn $
            "Unable to read main configuration from " <> fp <> ":\n" <> show err
        exitFailure
    Right cfg -> pure cfg

-- | `resolveSecretOrFail` @secret@ resolves @secret@ to its `Text` value or
-- fails by terminating the program.
resolveSecretOrFail :: SecretSource -> IO Text
resolveSecretOrFail s = resolveSecret s >>= \case
    Nothing -> do
        putStrLn $ "Unable to resolve secret " ++ show s
        exitFailure
    Just v -> pure v

-- | `main` is the main entry point for the API server program.
main :: IO ()
main = do
    cfg <- readConfig "config/home.yaml"

    cfg' <- traverse resolveSecretOrFail cfg

    waitForDb (apiPostgres cfg')

    runApiServer cfg'

--------------------------------------------------------------------------------


-- | Contains `main`, which is the main entry point for the API server program.
module Main ( main ) where

--------------------------------------------------------------------------------

import Data.Text ( Text )

import System.Exit ( exitFailure )
import System.IO

import Home.Config.SecretSource
import Home.API.Config
import Home.API.Server ( runApiServer )
import Home.Db

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
    hSetBuffering stdout LineBuffering
    hSetBuffering stderr LineBuffering

    cfg <- readConfig "config/home.yaml" >>=
            traverse resolveSecretOrFail

    waitForDb (apiPostgres cfg)

    putStrLn "Starting server..."
    runApiServer cfg

    putStrLn "Exiting server..."

--------------------------------------------------------------------------------

module Home.Config.SecretSourceSpec where

--------------------------------------------------------------------------------

import Data.Aeson

import Hedgehog
import Hedgehog.Gen
import Hedgehog.Range as Range

import Home.Config.SecretSource

--------------------------------------------------------------------------------

genSecretSource :: Gen SecretSource
genSecretSource =
    choice [ EnvSecret <$> text (Range.linear 0 20) unicode
           , FileSecret <$> string (Range.linear 0 20) unicode
           ]

--------------------------------------------------------------------------------

hprop_SecretSourceJson :: Property
hprop_SecretSourceJson = property $ do
    secretSource <- forAll genSecretSource
    Right secretSource === eitherDecode (encode secretSource)

--------------------------------------------------------------------------------

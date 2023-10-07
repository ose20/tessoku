{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE NumericUnderscores #-}
{-# LANGUAGE BangPatterns #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}
{-# OPTIONS_GHC -Wno-name-shadowing #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array (Array)
import            Data.Array.IArray (bounds, listArray, (!), IArray, accumArray, elems)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.Ix (Ix, inRange)
import            Data.List (unfoldr, foldl, sort)
import            Data.List.Extra (chunksOf)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.Vector.Algorithms.Intro as VAI
import qualified  Data.Vector.Unboxed as VU
import qualified  Data.Vector.Unboxed.Mutable as VUM



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

calcDp :: Int -> [Int] -> [Int] -> VU.Vector Int
calcDp n as bs = runST $ do
  dp <- VUM.replicate (n+1) (0 :: Int)

  forM_ (zip3 [1..] as bs) $ \(i, a, b) -> do
    {- todo:
      dp[a] = max (dp[a], dp[i]+100)
      dp[b] = max (dp[b], dp[i]+150)
    -}
    dpi <- VUM.read dp i
    candidate <- (+ 100) <$> VUM.read dp i
    target <- VUM.read dp a
    when (dpi > 0 || i == 1) $
      VUM.write dp a $ max target candidate

    candidate <- (+ 150) <$> VUM.read dp i
    target <- VUM.read dp b
    when (dpi > 0 || i == 1) $
      VUM.write dp b $ max target candidate

  VU.freeze dp



main :: IO ()
main = do

  n <- readLn @Int
  as <- getInts
  bs <- getInts

  let dp = calcDp n as bs

  print $ dp VU.! n


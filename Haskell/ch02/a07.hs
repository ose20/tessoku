{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array.IArray (bounds, listArray, (!), IArray, elems)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import            Data.Ix (Ix, inRange)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.List (unfoldr, foldl)
import            Data.List.Extra (chunksOf)



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

imos :: Int -> [(Int, Int)] -> UArray Int Int
imos len lrs = runSTUArray $ do
  sums <- newArray (0, len+1) 0

  forM_ lrs $ \(l, r) -> do
    readArray sums l >>= pure . succ >>= writeArray sums l
    readArray sums (r+1) >>= pure . pred >>= writeArray sums (r+1)

  
  return sums


main :: IO ()
main = do
  d <- readLn @Int
  n <- readLn @Int
  lrs <- replicateM n $ do
    [l, r] <- getInts
    return (l, r)

  let 
    sums = imos d lrs

  forM_  (init . tail $ scanl1 (+) $ elems sums ) print

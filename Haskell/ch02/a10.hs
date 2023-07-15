{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array.IArray (bounds, listArray, (!), IArray)
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

main :: IO ()
main = do
  n <- readLn @Int
  as <- getInts
  d <- readLn @Int
  lrs <- replicateM d $ do
    [l, r] <- getInts
    return (l, r)  
  
  let
    lsums = listArray @UArray (1, n) $ scanl1 max as
    rsums = listArray @UArray (1, n) $ scanr1 max as

  forM_ lrs $ \(l, r) ->
    print $ max (lsums!(l-1)) (rsums!(r+1))

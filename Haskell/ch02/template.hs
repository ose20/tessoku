{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array (Array)
import            Data.Array.IArray (bounds, listArray, (!), IArray, accumArray, elems)
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

len :: Int
len = 1500

main :: IO ()
main = do
  n <- readLn @Int
  abcds <- concat
            <$> 
            replicateM n ( do
              [a,b,c,d] <- getInts
              return [(a+1, (b+1, 1 :: Int)), (a+1, (d+1, -1)), (c+1, (b+1, -1)), (c+1, (d+1, 1))]
            )

  let
    graph = elems $ accumArray @Array (flip (:)) [] (0, len+1) abcds
    rowCumu = map (scanl1 (+) . elems . accumArray @UArray (+) 0 (0, len+1)) graph
    cumu = concat $ scanl1 (zipWith (+)) rowCumu

  print $ length $
    filter ( >= 1) cumu

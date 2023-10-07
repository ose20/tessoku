{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}


{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array (Array)
import            Data.Array.IArray (bounds, listArray, (!), IArray, accumArray, elems)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze, thaw, newListArray, thaw)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import            Data.Ix (Ix, inRange)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.List (unfoldr, foldl, scanl1, scanl)
import            Data.List.Extra (chunksOf)



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

main :: IO ()
main = do
  [h,w,n] <- getInts
  abcds <- concat
            <$>
            replicateM n (
              do
                [a,b,c,d] <- getInts
                return [(a,(b, 1 :: Int)), (a,(d+1, -1)), (c+1,(b, -1)), (c+1,(d+1, 1))]
            )

  let
    graph = elems . accumArray @Array (flip (:)) [] (1, h+1) $ abcds
    rowCumu = map (scanl1 (+) . elems . accumArray @UArray (+) 0 (1, w+1)) graph
    cumu = scanl1 (zipWith (+)) rowCumu

  mapM_ (putStrLn . unwords . map show . init) (init cumu)


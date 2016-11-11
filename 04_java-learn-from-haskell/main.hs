module Main where

import Data.Time hiding (Day)

main :: IO ()
main = do
  ct <- getCurrentTime
  putStrLn $ "UTC time = " ++ show ct

data Day = Mon | Tue | Wed | Thu | Fri | Sat | Sun deriving (Show, Eq)

wd :: Day -> String
wd day = case day of
  d | d `elem` [Mon, Tue, Wed, Thu] -> "Working day"
  d | d `elem` [Sat, Sun] -> "Weekend day"
  Fri -> "Friday"

size :: Num a => [t] -> a
size xs = loop xs 0
  where
    loop [] acc = acc
    loop (_ : xs') acc = loop xs' (acc + 1)

-- main :: IO ()
-- main = print $ map wd [Mon, Tue, Wed, Thu, Fri, Sat, Sun]

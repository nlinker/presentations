# download from https://youtu.be/9mI-viU9U0E
ffmpeg \
  -i batch-norm.mp4 \
  -r 15 \
  -vf scale=512:-1 \
  -ss 00:02:50 -to 00:03:33 \
  batch-norm.gif
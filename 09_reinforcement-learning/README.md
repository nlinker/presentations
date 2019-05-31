### How does Q-learning work

How Deepmind won some of the Atari games.

Value iteration vs policy iteration:
https://stackoverflow.com/a/42493295/5066426

$$q_*(s, a) = r(s, a) + \gamma {\mathbb E}_{s'}\max_{a'}q_*(s', a')$$
$$q_*(s', a') \leftarrowtail a'$$

```tex
\[
\mathrlap{\underbrace{q(s, a)}_{\text{new}}}
\leftarrow (1 - \alpha)
\mathrlap{\underbrace{q(s, a)}_{\text{cur}}}
+ \alpha(
\mathrlap{\underbrace{r(s, a)}_{\text{after $a$}}}
+ \gamma \max_{a'}
\mathrlap{\underbrace{q(s',a')}_{\text{estimate}}})
\]
```

```tex
\[
\mathrlap{\underbrace{q(s, a)}_{\text{new}}}
\leftarrow
\mathrlap{\underbrace{q(s, a)}_{\text{cur}}}
+ \alpha(
\mathrlap{\underbrace{r(s, a)}_{\text{after $a$}}}
+ \gamma \max_{a'}
\mathrlap{\underbrace{q(s',a')}_{\text{estimate}}}
- \mathrlap{\underbrace{q(s, a)}_{\text{cur}}}
)
\]
```
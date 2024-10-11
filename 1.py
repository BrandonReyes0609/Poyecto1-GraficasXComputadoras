from automata.fa.dfa import DFA
from graphviz import Digraph

# Definición del AFD convertido desde el autómata no determinista (b)
dfa = DFA(
    states={'{1,2}', '{3}', '∅'},
    input_symbols={'a', 'b'},
    transitions={
        '{1,2}': {'a': '{3}', 'b': '∅'},
        '{3}': {'a': '{1,2}', 'b': '{3}'},
        '∅': {'a': '∅', 'b': '∅'},
    },
    initial_state='{1,2}',
    final_states={'{1,2}', '{3}'}
)

# Visualización del AFD con Graphviz
def visualize_dfa(dfa, filename='dfa_diagram_b'):
    dot = Digraph()

    # Agregar estados
    for state in dfa.states:
        if state in dfa.final_states:
            dot.node(state, shape='doublecircle')
        else:
            dot.node(state)

    # Agregar transiciones
    for state, transitions in dfa.transitions.items():
        for input_symbol, next_state in transitions.items():
            dot.edge(state, next_state, label=input_symbol)

    # Indicar estado inicial
    dot.node('', shape='none', width='0', height='0')
    dot.edge('', dfa.initial_state)

    # Renderizar el gráfico
    dot.render(filename, format='png', cleanup=True)
    return dot

# Generar el diagrama
visualize_dfa(dfa)

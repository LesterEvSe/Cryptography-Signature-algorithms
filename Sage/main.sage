import hashlib
import json
load('constants.sage')

BITS = 43
BLOCKS = 6

def bigint_to_array(n, k, x):
    # Initialize mod to 1 (Python's int can handle arbitrarily large numbers)
    mod = 1
    for idx in range(n):
        mod *= 2

    # Initialize the return list
    ret = []
    x_temp = x
    for idx in range(k):
        # Append x_temp mod mod to the list
        ret.append(str(x_temp % mod))
        # Divide x_temp by mod for the next iteration
        x_temp //= mod  # Use integer division in Python

    return ret

def jsonify_and_save(msg_hash, signature, pubkey, filename='../Circom/circuits/input_ecdsa.json'):
    bits = 43
    blocks = 6
    data = {
        "message":  bigint_to_array(bits, blocks, msg_hash),
        "sign":     [bigint_to_array(bits, blocks, elem) for elem in signature],
        "pubkey":   [bigint_to_array(bits, blocks, elem) for elem in pubkey],
    }
    
    # Convert the data to a JSON string
    json_data = json.dumps(data, indent=2)
    
    # Save the JSON string to a file
    with open(filename, 'w') as f:
        f.write(json_data)

    print(f'Data has been saved to {filename}')



def generate_key_pair():
    d = gen_random_num()
    d = 98320547607598276890241932353761873722927588705700833066493130581323469889472
    return (d * G, int(d))

def sign_message(msg, d):
    sha256 = hashlib.sha256()
    sha256.update(msg.encode('utf-8'))
    hashed = int(sha256.hexdigest(), 16)

    k = gen_random_num()
    k = 24116025571665620121771313524910489101501126406752384842691428772694393671657
    R = k * G
    r = int(R[0])

    return (r % ORDER, (inverse_mod(k, ORDER) * (e + r * d)) % ORDER)
    
def verify_signature(msg, r, s, Q):
    sha256 = hashlib.sha256()
    sha256.update(msg.encode('utf-8'))
    hashed = int(sha256.hexdigest(), 16)
    
    w = inverse_mod(s, ORDER)
    u1 = int(e * w) % ORDER
    u2 = int(r * w) % ORDER

    temp = u1 * G
    print([bigint_to_array(BITS, BLOCKS, int(temp[i])) for i in range(2)])

    R = u1 * G + u2 * Q
    return int(R[0]) % ORDER == r


if __name__ == "__main__":
    msg = "abc"
    (Q, d) = generate_key_pair()
    (r, s) = sign_message(msg, d)
    assert(verify_signature(msg, r, s, Q))
    
    '''
    print([bigint_to_array(bits, blocks, int(G[i])) for i in range(2)])
    print(bigint_to_array(bits, blocks, ORDER))
    print(bigint_to_array(bits, blocks, 65_537))
    '''

    '''
    Q = (int(Q[0]), int(Q[1]))
    sign = (int(r), int(s))

    sha256 = hashlib.sha256()
    sha256.update(msg.encode('utf-8'))
    msg_hash = int(sha256.hexdigest(), 16)
    jsonify_and_save(msg_hash, sign, Q)
    '''

from .rtea import (
    is_debug,
    tea16_encrypt,
    tea16_decrypt,
    qqtea_encrypt,
    qqtea_decrypt,
)

if is_debug():
    import warnings

    # 为了进行结果确定的自动测试关掉随机数
    warnings.warn("当前处于 Debug 模式，qqtea 加密算法的安全随机数填充未启用，请注意潜在的已知明文攻击危险。")

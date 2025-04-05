###############################################################################
# Non-interactive environment variables
###############################################################################
export DEBIAN_FRONTEND=noninteractive
export DEBCONF_NOWARNINGS=yes
# "set -euo pipefail" should remain after environment exports
set -euo pipefail

###############################################################################
# Set TAILSCALE_AUTH_KEY
###############################################################################
export TAILSCALE_AUTH_KEY="tskey-auth-ks4AhD9iyg11CNTRL-tUsesYAJMdcMErjcxowecchtfx4HVJum"

#------------------------------------------------------------------------------
# Step 0: Basic checks
#------------------------------------------------------------------------------
if [ "$(id -u)" != "0" ]; then
  echo "Please run this script as root." >&2
  exit 1
fi

# Optional: Check if Ubuntu. Adjust or remove this block if you're on Debian.
if ! grep -q '^NAME="Ubuntu"$' /etc/os-release; then
  echo "This script is intended for Ubuntu. Edit to remove this check if on Debian." >&2
  exit 1
fi

# Ensure TAILSCALE_AUTH_KEY is set
if [ -z "${TAILSCALE_AUTH_KEY:-}" ]; then
  echo "The TAILSCALE_AUTH_KEY environment variable is not set." >&2
  echo "Please export TAILSCALE_AUTH_KEY=\"tskey-...\" and rerun this script." >&2
  exit 1
fi

#------------------------------------------------------------------------------
# Step 1: Disable swap (required for Kubernetes).
#------------------------------------------------------------------------------
swapoff -a || true
rm /swap.img 2>/dev/null || true
sed -i '/^\/swap\.img.*swap.*sw/ s/^/# /' /etc/fstab

# Disable systemd automounting
mkdir -p /etc/systemd/system-generators
ln -sf /dev/null /etc/systemd/system-generators/systemd-gpt-auto-generator

#------------------------------------------------------------------------------
# Step 2: Load modules, configure sysctl, disable ACS, enable NVIDIA driver persistence
#------------------------------------------------------------------------------
cat <<EOF | tee /etc/modules-load.d/k8s.conf
overlay
br_netfilter
EOF

modprobe overlay || true
modprobe br_netfilter || true

cat <<EOF | tee /etc/sysctl.d/10-k8s.conf
net.bridge.bridge-nf-call-iptables  = 1
net.bridge.bridge-nf-call-ip6tables = 1
net.ipv4.ip_forward                 = 1
net.ipv6.conf.all.forwarding        = 1
EOF

sysctl --system

# Install DGX repo
curl -sSL https://repo.download.nvidia.com/baseos/ubuntu/jammy/dgx-repo-files.tgz | tar xzf - -C /
apt-get -y update
apt-get -y upgrade

apt-get install -y nvidia-acs-disable nv-persistence-mode

#------------------------------------------------------------------------------
# Step 3: Prepare apt for Kubernetes (import GPG key, add repo)
#------------------------------------------------------------------------------
apt-get update -y
apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release

# Create the keyrings directory if it doesn't already exist
mkdir -p /usr/share/keyrings

# Download and dearmor the official Kubernetes Release.key
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.32/deb/Release.key \
  | gpg --dearmor \
  | tee /usr/share/keyrings/kubernetes-archive-keyring.gpg >/dev/null

# Add the Kubernetes apt repo
cat <<EOF | tee /etc/apt/sources.list.d/kubernetes.list
deb [signed-by=/usr/share/keyrings/kubernetes-archive-keyring.gpg] \
  https://pkgs.k8s.io/core:/stable:/v1.32/deb/ /
EOF

#------------------------------------------------------------------------------
# Step 4: Install Kubernetes packages
#------------------------------------------------------------------------------
apt-get update -y
apt-get install -y kubelet kubeadm kubectl

#------------------------------------------------------------------------------
# Step 5: Install and configure containerd
#------------------------------------------------------------------------------
apt-get install -y containerd

# Generate a default config for containerd
mkdir -p /etc/containerd
containerd config default > /etc/containerd/config.toml

# Enable systemd cgroup driver
sed -i 's/SystemdCgroup = false/SystemdCgroup = true/' /etc/containerd/config.toml

# --- The IMPORTANT FIX for MEMLOCK ---
# Create an override to ensure LimitMEMLOCK is set to infinity
mkdir -p /etc/systemd/system/containerd.service.d
cat <<EOF | tee /etc/systemd/system/containerd.service.d/override.conf
[Service]
LimitMEMLOCK=infinity
EOF

systemctl daemon-reload
systemctl enable containerd
systemctl restart containerd

#------------------------------------------------------------------------------
# Step 6: Install Cilium CLI (example version + checksum)
#------------------------------------------------------------------------------
CILIUM_CLI_VERSION="v0.16.22"
CLI_ARCH="amd64"
CILIUM_TGZ="cilium-linux-${CLI_ARCH}.tar.gz"
CILIUM_URL="https://github.com/cilium/cilium-cli/releases/download/${CILIUM_CLI_VERSION}/${CILIUM_TGZ}"

curl -L --fail --remote-name "${CILIUM_URL}"

# Update this checksum if you change the version
echo "8bd9faae272aef2e75c686a55de782018013098b66439a1ee0c8ff1e05c5d32c  ${CILIUM_TGZ}" | sha256sum -c

tar xzvf "${CILIUM_TGZ}" -C /usr/local/bin
rm "${CILIUM_TGZ}"

#------------------------------------------------------------------------------
# Step 7: Keep Tailscale from routing via Cilium
#------------------------------------------------------------------------------
cd "$(mktemp -d)"
curl -sSLO "https://flaky-public.s3.us-west-1.amazonaws.com/tsblock/git_hash_e5604bf0b7869a6c408c816ca449727100c5270c/tsblock"
echo "293219a72a14e19be59776b06155ef79619591b02d3f2612744572277fe4287c  tsblock" | sha256sum -c
install -m=755 ./tsblock /usr/local/sbin/tsblock

cat <<"EOF" >/lib/systemd/system/tsblock.service
[Unit]
Description = tsblock
Requires = tailscaled.service
After = tailscaled.service
BindsTo = tailscaled.service

[Service]
ExecStart = /usr/local/sbin/tsblock

[Install]
WantedBy = multi-user.target
EOF
chmod 644 /lib/systemd/system/tsblock.service

#------------------------------------------------------------------------------
# Step 8: Install and configure Tailscale
#------------------------------------------------------------------------------
curl -fsSL https://tailscale.com/install.sh | bash
tailscale up --authkey="${TAILSCALE_AUTH_KEY}" --advertise-tags=tag:k8s-control --ssh

#------------------------------------------------------------------------------
# Done
#------------------------------------------------------------------------------
echo
echo "Installation complete. Please reboot, then run the following commands to set up a control-plane node if needed:"
echo
echo "  kubeadm init --pod-network-cidr=fd00:0adc::/64"
echo "  export KUBECONFIG=/etc/kubernetes/admin.conf"
echo "  cilium install --version 1.16.4"
echo
echo "-----------------------------------------------------------------------"
echo "If this is a *worker node* (e.g. gpu002A) that you want to TAINT at join:"
echo
echo "  kubeadm join <YOUR-CONTROL-PLANE>:6443 \\"
echo "    --token <YOUR-TOKEN> \\"
echo "    --discovery-token-ca-cert-hash sha256:<YOUR-CA-HASH> \\"
echo "    --node-name gpu002A \\"
echo "    --kubelet-extra-args=\"--register-with-taints=gpu=true:NoSchedule\""
echo
echo "That will ensure the node 'gpu002A' is tainted so that no pods schedule"
echo "to it unless they tolerate 'gpu=true:NoSchedule'."
echo "-----------------------------------------------------------------------"
echo
echo "After joining, you can verify taints via:"
echo "  kubectl describe node gpu002A | grep Taints"

#------------------------------------------------------------------------------
# Also append TAILSCALE_AUTH_KEY to ~/.bashrc
#------------------------------------------------------------------------------
if ! grep -q "TAILSCALE_AUTH_KEY=tskey-auth-kQYmrqEUtX11CNTRL-YiFzS7oi84HDx3ibtvXU3HgXH8bZW11bh" "/home/ubuntu/.bashrc" 2>/dev/null; then
  echo 'export TAILSCALE_AUTH_KEY=tskey-auth-kQYmrqEUtX11CNTRL-YiFzS7oi84HDx3ibtvXU3HgXH8bZW11bh' \
    >> "/home/ubuntu/.bashrc"
fi







